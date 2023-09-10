import { useState } from "react";
import "./App.css";

const NUM_ROW = 20;
const NUM_COL = 7;
const NUM_BARS = NUM_COL * 2;
const NUM_LEDS = 10;

const App = () => {
  const [matrix, setMatrix] = useState(
    Array.from({ length: NUM_ROW }, () =>
      Array.from({ length: NUM_COL }, () => false)
    )
  );

  /**
   * handle click led event
   * @param i row
   * @param j col
   */
  const handleClick = (i: number, j: number) => {
    setMatrix((prevMatrix) => {
      const newMatrix = JSON.parse(JSON.stringify(prevMatrix));
      newMatrix[i][j] = !newMatrix[i][j];
      return newMatrix;
    });
  };

  /**
   * set class name of led
   * @param row
   * @param led
   * @returns class name
   */
  const ledClass = (row: number, led: boolean) => {
    if (led) {
      if (5 <= row && row < 15) {
        return "led led-green";
      } else if (2 <= row && row < 18) {
        return "led led-yellow";
      } else {
        return "led led-red";
      }
    } else {
      return "led led-off";
    }
  };

  /**
   *
   * @returns code block
   */
  const codeBlock = () => {
    const code = [];
    code.push(<span key={0}>{"const uint16_t data[NUM_BARS] = {"}</span>);
    for (let i = 0; i < NUM_BARS; i++) {
      let data = i << 10;
      for (let j = 0; j < NUM_LEDS; j++) {
        const c = i % 7;
        const lr = Math.trunc(i / NUM_COL);
        const r = NUM_LEDS - 1 + (2 * lr - 1) * j + lr;
        if (matrix[r][c]) {
          data += 1 << j;
        }
      }
      code.push(
        <span key={i + 1}>
          &nbsp;&nbsp;
          {`0x${data.toString(16).padStart(4, "0").toUpperCase()}, `}
        </span>
      );
    }
    code.push(<span key={NUM_BARS + 1}>{"};"}</span>);
    return code;
  };

  const copyToClipboard = () => {
    const code = codeBlock()
      .map((c) => c.props.children)
      .join("\n");

    globalThis.navigator.clipboard.writeText(code);
  };

  return (
    <>
      <header>
        <h1>LED Matrix Generator</h1>
      </header>

      <div className="display-panel">
        <table className="led-matrix">
          <tbody>
            {matrix.map((row, i) => (
              <tr key={i}>
                {row.map((col, j) => (
                  <td
                    key={j}
                    className={ledClass(i, col)}
                    onClick={() => {
                      handleClick(i, j);
                    }}
                  ></td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="codeblock">
        <code>{codeBlock()}</code>
      </div>
      <button className="copy-button" onClick={copyToClipboard}>
        copy to clipboard &#x2398;
      </button>

      <footer>
        <p>&copy; 2023 shimo8810 All Rights Reserved.</p>
      </footer>
    </>
  );
};

export default App;
