import mermaid from "mermaid";
import { useEffect } from "react";
import Panzoom, { PanzoomObject } from "@panzoom/panzoom";

mermaid.initialize({});

const Mermaid = ({
  chart,
  id,
  onChange,
}: {
  chart: string;
  id: string;
  onChange?: (value: PanzoomObject) => void;
}) => {
  useEffect(() => {
    document.getElementById(id)?.removeAttribute("data-processed");

    mermaid.render("mySvgId", chart).then((res) => {
      const container = document.getElementById(id);
      if (container && container.childElementCount === 0) {
        container.innerHTML = res.svg;
        const svg = container.querySelector("svg");
        if (svg) {
          const panzoom = Panzoom(svg);
          onChange?.(panzoom);
        }
      }
    });
  }, [chart]);

  return (
    <div
      className="mermaid"
      id={id}
      style={{
        height: "100%",
      }}
    ></div>
  );
};

export default Mermaid;
