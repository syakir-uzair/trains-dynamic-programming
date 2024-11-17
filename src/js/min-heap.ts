import {Route} from './types';

export class MinHeap {
  public heap: Route[];

  constructor() {
    this.heap = [];
  }

  // Helper Methods
  getLeftChildIndex(parentIndex: number) {
    return 2 * parentIndex + 1;
  }
  getRightChildIndex(parentIndex: number) {
    return 2 * parentIndex + 2;
  }
  getParentIndex(childIndex: number) {
    return Math.floor((childIndex - 1) / 2);
  }
  hasLeftChild(index: number) {
    return this.getLeftChildIndex(index) < this.heap.length;
  }
  hasRightChild(index: number) {
    return this.getRightChildIndex(index) < this.heap.length;
  }
  hasParent(index: number) {
    return this.getParentIndex(index) >= 0;
  }
  leftChild(index: number) {
    return this.heap[this.getLeftChildIndex(index)];
  }
  rightChild(index: number) {
    return this.heap[this.getRightChildIndex(index)];
  }
  parent(index: number) {
    return this.heap[this.getParentIndex(index)];
  }

  swap(indexOne: number, indexTwo: number) {
    const temp = this.heap[indexOne];
    this.heap[indexOne] = this.heap[indexTwo];
    this.heap[indexTwo] = temp;
  }

  peek() {
    if (this.heap.length === 0) {
      return null;
    }
    return this.heap[0];
  }

  remove() {
    if (this.heap.length === 0) {
      return null;
    }
    const route = this.heap[0];
    this.heap[0] = this.heap[this.heap.length - 1];
    this.heap.pop();
    this.heapifyDown();
    return route;
  }

  add(route: Route) {
    this.heap.push(route);
    this.heapifyUp();
  }

  heapifyUp() {
    let index = this.heap.length - 1;
    while (
      this.hasParent(index) &&
      this.parent(index).distance > this.heap[index].distance
    ) {
      this.swap(this.getParentIndex(index), index);
      index = this.getParentIndex(index);
    }
  }

  heapifyDown() {
    let index = 0;
    while (this.hasLeftChild(index)) {
      let smallerChildIndex = this.getLeftChildIndex(index);
      if (
        this.hasRightChild(index) &&
        this.rightChild(index).distance < this.leftChild(index).distance
      ) {
        smallerChildIndex = this.getRightChildIndex(index);
      }
      if (this.heap[index].distance < this.heap[smallerChildIndex].distance) {
        break;
      } else {
        this.swap(index, smallerChildIndex);
      }
      index = smallerChildIndex;
    }
  }
}
