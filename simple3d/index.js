const BACKGROUD = "#C0C0C0"
const FOREGROUD = "#8000FF"

console.log(game)
game.width = game.height = 800
const ctx = game.getContext("2d")
console.log(ctx)

function clear() {
    ctx.fillStyle = BACKGROUD
    ctx.fillRect(0, 0, game.width, game.height)
}

function point({ x, y }) {
    const SIZE = 10
    ctx.fillStyle = FOREGROUD
    ctx.fillRect(x - SIZE / 2, y - SIZE / 2, SIZE, SIZE)
}

function line(p1, p2) {
    ctx.lineWidth = 3
    ctx.strokeStyle = FOREGROUD
    ctx.beginPath()
    ctx.moveTo(p1.x, p1.y)
    ctx.lineTo(p2.x, p2.y)
    ctx.stroke()
}

// 将[-1, 1]范围的坐标映射到画布上
function screen({ x, y }) {
    return {
        x: (x + 1) / 2 * game.width,
        y: (1 - y) / 2 * game.height
    }
}

// 三维物体映射到二维平面
function project({ x, y, z }) {
    return {
        x: x / z,
        y: y / z
    }
}

// 关于y轴旋转
function rotateXz({ x, y, z }, angle) {
    const cos = Math.cos(angle)
    const sin = Math.sin(angle)
    return {
        x: x * cos - z * sin,
        y,
        z: x * sin + z * cos,
    }
}

// 修改点的远近
function translateZ({ x, y, z }, dz) {
    return {
        x, y, z: z + dz
    }
}

const vs = [
    { x: 0.25, y: 0.25, z: 0.25 },
    { x: 0.25, y: -0.25, z: 0.25 },
    { x: -0.25, y: -0.25, z: 0.25 },
    { x: -0.25, y: 0.25, z: 0.25 },

    { x: 0.25, y: 0.25, z: -0.25 },
    { x: 0.25, y: -0.25, z: -0.25 },
    { x: -0.25, y: -0.25, z: -0.25 },
    { x: -0.25, y: 0.25, z: -0.25 },
]

const fs = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
]

const FPS = 60
const dt = 1 / FPS
let dz = 1
let angle = 0

function updateDz() {
    const SPEED = 1
    dz += SPEED * dt
}

function updateAngle() {
    const SPEED = 1 / 4
    angle += 2 * Math.PI * dt * SPEED
    angle %= 2 * Math.PI
}

function printVs() {
    for (const v of vs) {
        point(screen(project(translateZ(rotateXz(v, angle), dz))))
    }
}

function printFs() {
    for (const f of fs) {
        for (let i = 0; i < f.length; i++) {
            const p1 = vs[f[i]]
            const p2 = vs[f[(i + 1) % f.length]]
            line(
                screen(project(translateZ(rotateXz(p1, angle), dz))),
                screen(project(translateZ(rotateXz(p2, angle), dz))),
            )
        }
    }
}

function frame() {
    clear()
    // updateDz()
    updateAngle()
    // printVs()
    printFs()
}

function timeoutFrame() {
    frame()
    setTimeout(timeoutFrame, dt)
}

function animationFrame() {
    frame()
    requestAnimationFrame(animationFrame)
}

animationFrame()
// timeoutFrame()
