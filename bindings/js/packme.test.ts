import { expect, test } from "bun:test";
import { init } from "./packme";
import { AlgoInput } from "./interface";

const f = Bun.file("./packme.wasm");
const buf = await f.arrayBuffer();
const { pack, version } = await init(buf);

test("Version Test", () => {
  expect(typeof version()).toEqual("string");
});

test("Binding test", () => {
  const data: AlgoInput = {
    containers: [
      {
        id: "container 1",
        qty: 1,
        dim: [20, 20, 30],
      },
    ],
    items: [
      {
        id: "item 1",
        qty: 5,
        dim: [10, 10, 30],
      },
    ],
  };

  const result = pack(data);
  expect(result.unpacked_items.length).toEqual(1);
  expect(result.containers[0].items.length).toEqual(4);
});
