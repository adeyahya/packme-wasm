export type ItemInput = {
  id: string;
  qty: number;
  dim: [number, number, number];
};

export type AlgoInput = {
  containers: ItemInput[];
  items: ItemInput[];
};

export type Rotation = "LWH" | "WLH" | "WHL" | "HLW" | "HWL" | "LHW";

export type Vector3 = {
  length: number;
  width: number;
  height: number;
};

export type ItemResult = {
  id: string;
  dim: Vector3;
  pos: Vector3;
  rot: Rotation;
};

export type ContainerResult = {
  id: string;
  dim: Vector3;
  items: ItemResult[];
};

export type AlgoResult = {
  containers: ContainerResult[];
  unpacked_items: ItemResult[];
};
