import { makeItem } from "@/test/factories";
import { useImagesStore } from "./images";

beforeEach(() => useImagesStore.getState().clear());

describe("useImagesStore", () => {
  it("agrega sin duplicar y devuelve cuántas son nuevas", () => {
    const { addItems } = useImagesStore.getState();
    expect(addItems([makeItem(1), makeItem(2)], [])).toBe(2);
    expect(addItems([makeItem(2), makeItem(3)], [])).toBe(1);
    expect(useImagesStore.getState().entries).toHaveLength(3);
  });

  it("nuevas entradas inician en cola", () => {
    useImagesStore.getState().addItems([makeItem(1)], []);
    expect(useImagesStore.getState().entries[0].status).toBe("queued");
  });

  it("quita y vacía", () => {
    const s = useImagesStore.getState();
    s.addItems([makeItem(1), makeItem(2)], [{ name: "x.gif", reason: "unsupported" }]);
    s.remove(makeItem(1).id);
    expect(useImagesStore.getState().entries).toHaveLength(1);
    s.clear();
    expect(useImagesStore.getState().entries).toHaveLength(0);
    expect(useImagesStore.getState().skipped).toHaveLength(0);
  });
});
