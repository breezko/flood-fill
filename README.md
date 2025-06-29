
# 🎨 Flood Fill Visualizer

A **WebAssembly-powered interactive flood fill visualization**, built in **Rust + HTML + JavaScript**.

Click any colored cell to start a **smart flood fill**:

* The clicked region fills with yellow.
* Any adjacent regions **smaller** than your growing yellow region are recursively swallowed.

Beautiful, responsive, and satisfying to watch!

---

## Live Showcase

`https://flood-fill.pages.dev`


## 📂 Project Structure

```
flood_fill/
├── src/
│   └── lib.rs           # Rust flood fill logic
├── pkg/                 # Generated WASM + JS glue
├── index.html           # HTML/JS frontend
└── README.md            # You are here
```

---

## ⚙️ Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

Make sure you have `rustc` and `cargo` in your PATH:

```bash
rustc --version
cargo --version
```

Install `wasm-pack` if you haven’t:

```bash
cargo install wasm-pack
```

---

## 🛠️ Build and Run

1. **Compile the Rust code to WebAssembly:**

   ```bash
   wasm-pack build --target web
   ```

   This will generate a `pkg/` folder containing:

   * `flood_fill_bg.wasm`
   * `flood_fill.js`
   * `package.json`

2. **Serve the project locally:**

   You can use any static HTTP server. For example:

   ```bash
   python3 -m http.server 8080
   ```

   Then open:

   ```
   http://localhost:8080
   ```

✅ You should see the grid and be able to click cells to fill them.

---

## 🧠 How It Works

* **Rust (WASM):**
  A BFS-based flood fill:

  * Fills the clicked region.
  * Recursively identifies and "eats" any smaller neighboring regions.
  * Returns all cells to fill in order.

* **JavaScript:**

  * Renders the grid dynamically based on the window size.
  * Animates filling cell by cell.
  * Provides a **Reset Grid** button to generate a new random layout.

* **HTML/CSS:**

  * Responsive grid layout.
  * Clean color styling (Red, Green, Blue, Yellow).

---

## 🎨 Customization Tips

* **Cell size:** Adjust `cellSize` in `index.html`.
* **Blob randomness:** Change the `Math.random() < 0.7` probability to grow bigger or smaller blobs.
* **Animation speed:** Tweak the `setTimeout` interval in `animate()`.

---

## 🙌 Acknowledgements

* [Rust and WebAssembly](https://rustwasm.github.io)
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

---

