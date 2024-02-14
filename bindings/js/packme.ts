import { AlgoInput, AlgoResult } from "./interface";

export const init = async (buffer: ArrayBuffer) => {
  const module = await WebAssembly.instantiate(buffer, {});

  const alloc = module.instance.exports.alloc as (len: number) => number;
  const dealloc = module.instance.exports.dealloc as (
    ptr: number,
    len: number
  ) => void;
  const _pack = module.instance.exports.pack as (
    ptr: number,
    len: number
  ) => number;

  const pack = (data: AlgoInput) => {
    const [ptr, len] = getStringPtr(JSON.stringify(data));
    const resultPtr = _pack(ptr, len);
    const [result, resultLen] = parseString(resultPtr);
    dealloc(ptr, len);
    dealloc(resultPtr, resultLen);
    return JSON.parse(result) as AlgoResult;
  };

  const getStringPtr = (strData: string) => {
    const encoder = new TextEncoder();
    const str = encoder.encode(strData);
    const ptr = alloc(str.length);
    const mem = new Uint8Array((module.instance.exports.memory as any).buffer);
    mem.set(str, ptr);
    return [ptr, str.length] as const;
  };

  const parseString = (ptr: number) => {
    const mem = new Uint8Array(
      (module.instance.exports.memory as any).buffer,
      ptr
    );
    let str = "";
    let len = 0;
    while (mem[len] !== 0) {
      str += String.fromCharCode(mem[len]);
      len++;
    }
    return [str, len] as const;
  };

  return {
    pack,
  };
};
