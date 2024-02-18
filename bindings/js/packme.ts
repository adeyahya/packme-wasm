import { AlgoInput, AlgoResult } from "./interface";
import { pack as _pack, __wbg_set_wasm } from "./bg";

export const init = async (buffer: ArrayBuffer) => {
  const module = await WebAssembly.instantiate(buffer, {});
  __wbg_set_wasm(module.instance.exports);

  const pack = (input: AlgoInput): AlgoResult => {
    const result = _pack(JSON.stringify(input));
    return JSON.parse(result) as AlgoResult;
  };

  return {
    pack,
  };
};
