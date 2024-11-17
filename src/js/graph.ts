import {MinHeap} from './min-heap';
import {Destination, Route} from './types';

export class Graph {
  public adjList: Map<string, Route[]>;
  public cache: Map<string, Map<string, Destination>>;

  constructor() {
    this.adjList = new Map();
    this.cache = new Map();
  }

  addEdge(from: string, to: string, distance: number) {
    const froms = this.adjList.get(from);
    const tos = this.adjList.get(to);
    if (froms) {
      froms.push({to, distance});
    } else {
      this.adjList.set(from, [{to, distance}]);
    }
    if (tos) {
      tos.push({to: from, distance});
    } else {
      this.adjList.set(to, [{to: from, distance}]);
    }
  }

  dijkstra(start: string): Map<string, Destination> {
    const destinations = new Map<string, Destination>();
    const minHeap = new MinHeap();
    const visited = new Set<string>();

    if (this.cache.has(start)) {
      return this.cache.get(start) ?? new Map();
    }

    this.adjList.forEach((_, to) => {
      destinations.set(start, {
        from: start,
        to,
        checkpoints: [],
        distance: 0,
        cumulativeDistance: Infinity,
      });
    });

    destinations.set(start, {
      from: start,
      to: start,
      checkpoints: [],
      distance: 0,
      cumulativeDistance: 0,
    });

    minHeap.add({to: start, distance: 0});

    while (minHeap.heap.length) {
      const min = minHeap.remove();

      if (!min) {
        throw new Error('Minimum distance not found');
      }

      const {to: current, distance: currentDistance} = min;

      if (visited.has(current)) {
        continue;
      }

      visited.add(current);

      const neighbors = this.adjList.get(current) || [];
      for (const {to: next, distance} of neighbors) {
        const newCumulativeDistance = currentDistance + distance;
        const destination = destinations.get(next);
        const prevDestination = destinations.get(current);
        const prevCheckpoints = prevDestination?.checkpoints ?? [];

        if (
          newCumulativeDistance < (destination?.cumulativeDistance ?? Infinity)
        ) {
          const checkpoints = prevDestination?.checkpoints.length
            ? [
                ...prevCheckpoints,
                {
                  to: current,
                  // since current distance is cumulative sum, deduct from final checkpoint distance
                  distance:
                    currentDistance -
                    prevCheckpoints[prevCheckpoints.length - 1].distance,
                },
              ]
            : current !== start
              ? [{to: current, distance: currentDistance}]
              : [];

          destinations.set(next, {
            from: start,
            to: next,
            checkpoints,
            distance,
            cumulativeDistance: newCumulativeDistance,
          });
          minHeap.add({to: next, distance: newCumulativeDistance});
        }
      }
    }

    this.cache.set(start, destinations);

    return destinations;
  }
  getDestination(from: string, to: string) {
    const destination = this.dijkstra(from).get(to);

    if (!destination) {
      throw new Error('Destination not found');
    }

    return destination;
  }
}
