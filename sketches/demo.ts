declare global {
  type Shapes = {
    rect: (x: number, y: number) => {}
  }
  var shapes: Shapes
}


export function main(time: number) {
    const n: number = 5000
    const t: number = time * 0.1
    for (let i: number = 0; i < n; i++) {
        let a: number = i / n
        let b: number = (a + t) % 1
        let x: number = Math.sin(b * Math.PI * 16) * 500 * a
        let y: number = Math.cos(b * Math.PI * 16) * 500 * a
        shapes.rect(x, y)
    }
}
