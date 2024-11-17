import {Graph} from './graph';
import {
  Destination,
  Input,
  Job,
  Movement,
  Output,
  Package,
  Train,
} from './types';

export class Navigation {
  public graph: Graph = new Graph();
  public trains: Map<string, Train> = new Map();
  public packages: Map<string, Package> = new Map();
  public cache: Map<string, Movement[]> = new Map();

  constructor(input: Input) {
    for (const edge of input.edges) {
      this.graph.addEdge(edge.from, edge.to, edge.distance);
    }

    for (const train of input.trains) {
      this.trains.set(train.name, {
        ...train,
        currentLocation: train.start,
        totalDistance: 0,
        packagesToPickUp: [],
        packagesPickedUp: [],
        packagesDelivered: [],
      });
    }

    for (const pack of input.packages) {
      this.packages.set(pack.name, {
        ...pack,
        toBePickedUpBy: '',
        pickedUpBy: '',
        deliveredBy: '',
      });
    }
  }

  getCacheKey(trains: Map<string, Train>, packages: Map<string, Package>) {
    const trainLocations: {train: string; location: string}[] = [];
    for (const [trainName, train] of trains) {
      trainLocations.push({
        train: trainName,
        location: train.currentLocation,
      });
    }
    trainLocations.sort((a, b) => {
      if (a.train > b.train) {
        return 1;
      } else if (a.train < b.train) {
        return -1;
      }
      return 0;
    });

    const packagesToBePickedUp: {package: string; train: string}[] = [];
    const packagesPickedUp: {package: string; train: string}[] = [];
    const packagesDelivered: {package: string; train: string}[] = [];
    for (const [packageName, pack] of packages) {
      if (pack.deliveredBy) {
        packagesDelivered.push({package: packageName, train: pack.deliveredBy});
      }
      if (pack.pickedUpBy) {
        packagesPickedUp.push({package: packageName, train: pack.pickedUpBy});
      }
      if (pack.toBePickedUpBy) {
        packagesToBePickedUp.push({
          package: packageName,
          train: pack.toBePickedUpBy,
        });
      }
    }

    packagesToBePickedUp.sort((a, b) => {
      if (a.package > b.package) {
        return 1;
      } else if (a.package < b.package) {
        return -1;
      }
      return 0;
    });

    packagesPickedUp.sort((a, b) => {
      if (a.package > b.package) {
        return 1;
      } else if (a.package < b.package) {
        return -1;
      }
      return 0;
    });

    packagesDelivered.sort((a, b) => {
      if (a.package > b.package) {
        return 1;
      } else if (a.package < b.package) {
        return -1;
      }
      return 0;
    });

    const trainLocationsCacheKey = trainLocations
      .map(item => `${item.train}:${item.location}`)
      .join(',');
    const packagesToBePickedUpCacheKey = packagesToBePickedUp
      .map(item => `${item.package}:${item.train}`)
      .join(',');
    const packagesPickedUpCacheKey = packagesPickedUp
      .map(item => `${item.package}:${item.train}`)
      .join(',');
    const packagesDeliveredCacheKey = packagesDelivered
      .map(item => `${item.package}:${item.train}`)
      .join(',');
    return `${trainLocationsCacheKey};${packagesToBePickedUpCacheKey};${packagesPickedUpCacheKey};${packagesDeliveredCacheKey}`;
  }

  cloneTrains(trains: Map<string, Train>): Map<string, Train> {
    const newTrains: Map<string, Train> = new Map();

    for (const [name, train] of trains) {
      newTrains.set(name, {
        ...train,
        packagesToPickUp: [...train.packagesToPickUp],
        packagesPickedUp: [...train.packagesPickedUp],
        packagesDelivered: [...train.packagesDelivered],
      });
    }

    return newTrains;
  }

  clonePackages(packages: Map<string, Package>): Map<string, Package> {
    const newPackages: Map<string, Package> = new Map();

    for (const [name, pack] of packages) {
      newPackages.set(name, {...pack});
    }

    return newPackages;
  }

  getOrFail<T>(name: string, map: Map<string, T>): T {
    const item = map.get(name);

    if (!item) {
      throw new Error(`${name} not found`);
    }

    return item;
  }

  getCapableTrains(
    pack: Package,
    packages: Map<string, Package>,
    trains: Map<string, Train>
  ): Train[] {
    const capableTrains: Train[] = [];

    for (const [, train] of trains) {
      const trainPackages = [
        ...train.packagesToPickUp,
        ...train.packagesPickedUp,
      ];
      const packagesTotalWeight = trainPackages.reduce((prev, cur) => {
        const pack = this.getOrFail<Package>(cur, packages);
        return prev + pack.weight;
      }, 0);
      const trainCapacityLeft = train.capacity - packagesTotalWeight;

      if (trainCapacityLeft >= pack.weight) {
        capableTrains.push(train);
      }
    }

    return capableTrains;
  }

  getPackagesToDeliver(
    train: Train,
    packages: Map<string, Package>,
    to: string
  ) {
    const packagesToDeliver: string[] = [];

    for (const packageName of [
      ...train.packagesToPickUp,
      ...train.packagesPickedUp,
    ]) {
      const pack = this.getOrFail<Package>(packageName, packages);
      if (pack.to === to) {
        packagesToDeliver.push(packageName);
        pack.pickedUpBy = '';
        pack.deliveredBy = train.name;
      }
    }

    if (packagesToDeliver.length) {
      train.packagesPickedUp = train.packagesPickedUp.filter(
        pack => !packagesToDeliver.includes(pack)
      );
      train.packagesDelivered = [
        ...train.packagesDelivered,
        ...packagesToDeliver,
      ];
    }

    return packagesToDeliver;
  }

  moveTrain(
    train: Train,
    destination: Destination,
    packages: Map<string, Package>,
    movements: Movement[]
  ): Movement[] {
    const checkpoints = destination.checkpoints;
    const newMovements = [...movements];
    const trainMovements = newMovements.filter(mv => mv.train === train.name);
    let startTime =
      // get existing time taken by the train
      trainMovements.length > 0
        ? trainMovements[trainMovements.length - 1].endTime
        : 0;
    let endTime =
      startTime +
      (checkpoints.length ? checkpoints[0].distance : destination.distance);

    if (destination.cumulativeDistance) {
      const newPackagesPickedUp: string[] = [...train.packagesToPickUp];
      if (newPackagesPickedUp.length) {
        // pick up package(s) scheduled in that location
        train.packagesPickedUp = [
          ...train.packagesPickedUp,
          ...newPackagesPickedUp,
        ];
        train.packagesToPickUp = [];

        for (const packageName of newPackagesPickedUp) {
          const pack = this.getOrFail<Package>(packageName, packages);
          pack.toBePickedUpBy = '';
          pack.pickedUpBy = train.name;
        }
      }

      const to = checkpoints.length ? checkpoints[0].to : destination.to;
      const packagesToDeliver = this.getPackagesToDeliver(train, packages, to);
      newMovements.push({
        startTime,
        endTime,
        train: train.name,
        from: destination.from,
        to,
        packagesPickedUp: newPackagesPickedUp,
        packagesDelivered: packagesToDeliver,
      });
      for (let i = 0; i < checkpoints.length; i++) {
        startTime = endTime;
        endTime =
          startTime +
          (i < checkpoints.length - 1
            ? checkpoints[i + 1].distance
            : destination.distance);
        const to =
          i < checkpoints.length - 1 ? checkpoints[i + 1].to : destination.to;
        const packagesToDeliver = this.getPackagesToDeliver(
          train,
          packages,
          to
        );
        newMovements.push({
          startTime,
          endTime,
          train: train.name,
          from: checkpoints[i].to,
          to,
          packagesPickedUp: [],
          packagesDelivered: packagesToDeliver,
        });
      }
      train.currentLocation = destination.to;
      train.totalDistance += destination.cumulativeDistance;
    }

    return newMovements;
  }

  getLongestDistanceInMovements(movements: Movement[]): number {
    let longestDistance = 0;
    for (const movement of movements) {
      if (movement.endTime > longestDistance) {
        longestDistance = movement.endTime;
      }
    }

    return longestDistance;
  }

  getNumberOfTrains(movements: Movement[]): number {
    const trains = new Set<string>();
    for (const movement of movements) {
      if (!trains.has(movement.train)) {
        trains.add(movement.train);
      }
    }
    return trains.size;
  }

  calculate(
    trains: Map<string, Train>,
    packages: Map<string, Package>,
    movements: Movement[]
  ): Movement[] {
    let minDistance = Infinity;
    let minTrains = Infinity;
    let bestMovements: Movement[] = movements;
    let totalCombinations = 0;
    const cacheKey = this.getCacheKey(trains, packages);
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey) ?? [];
    }

    const queue: Job[] = [];

    for (const [, pack] of packages) {
      if (pack.toBePickedUpBy || pack.pickedUpBy || pack.deliveredBy) {
        continue;
      }

      const capableTrains = this.getCapableTrains(pack, packages, trains);
      for (const train of capableTrains) {
        queue.push({
          trainName: train.name,
          packageName: pack.name,
          toPickUp: true,
        });
      }
    }

    for (const [, train] of trains) {
      for (const packageName of [
        ...train.packagesToPickUp,
        ...train.packagesPickedUp,
      ]) {
        queue.push({
          trainName: train.name,
          packageName,
          toPickUp: false,
        });
      }
    }

    for (const {trainName, packageName, toPickUp} of queue) {
      const newTrains = this.cloneTrains(trains);
      const newPackages = this.clonePackages(packages);
      const newTrain = this.getOrFail<Train>(trainName, newTrains);
      const newPackage = this.getOrFail<Package>(packageName, newPackages);
      const destination = toPickUp
        ? this.graph.getDestination(newTrain.currentLocation, newPackage.from)
        : this.graph.getDestination(newTrain.currentLocation, newPackage.to);
      const newMovements = this.moveTrain(
        newTrain,
        destination,
        newPackages,
        movements
      );

      if (toPickUp) {
        newTrain.packagesToPickUp = [
          ...newTrain.packagesToPickUp,
          newPackage.name,
        ];
        newPackage.toBePickedUpBy = newTrain.name;
      }

      const allMovements = this.calculate(newTrains, newPackages, newMovements);
      const longestTrainDistance =
        this.getLongestDistanceInMovements(allMovements);
      const numberOfTrains = this.getNumberOfTrains(allMovements);
      if (
        longestTrainDistance < minDistance ||
        (longestTrainDistance === minDistance && numberOfTrains < minTrains)
      ) {
        minDistance = longestTrainDistance;
        minTrains = numberOfTrains;
        bestMovements = [...allMovements];
      }
      totalCombinations++;
    }

    // If no combinations left, it could mean either:
    if (totalCombinations === 0) {
      const undeliveredPackages: string[] = [];
      for (const [, pack] of packages) {
        if (!pack.deliveredBy) {
          undeliveredPackages.push(pack.name);
        }
      }

      // All packages are delivered
      if (undeliveredPackages.length === 0) {
        return bestMovements;
        // Or something is wrong
      } else {
        throw new Error('No solution found');
      }
    }

    this.cache.set(cacheKey, bestMovements);
    return bestMovements;
  }

  solve(): Output {
    try {
      const movements = this.calculate(this.trains, this.packages, []);

      movements.sort((a, b) => {
        if (a.train > b.train) {
          return 1;
        } else if (a.train < b.train) {
          return -1;
        } else if (a.startTime > b.startTime) {
          return 1;
        } else if (a.startTime < b.startTime) {
          return -1;
        }
        return 0;
      });

      return movements.map(mv => ({
        W: mv.startTime,
        T: mv.train,
        N1: mv.from,
        P1: mv.packagesPickedUp,
        N2: mv.to,
        P2: mv.packagesDelivered,
      }));
    } catch (e) {
      // console.log(e);

      return [];
    }
  }
}
