import { FileNode } from "@/components/file-tree";

export interface Project {
  id: number;
  name: string;
  path: string;
  last_opened: number;
}

export const sampleStructure: FileNode = {
  name: "src",
  path: "/src",
  type: "folder",
  children: [
    {
      name: "main",
      path: "/src/main",
      type: "folder",
      children: [
        {
          name: "java",
          path: "/src/main/java",
          type: "folder",
          children: [
            {
              name: "com.example",
              path: "/src/main/java/com/example",
              type: "folder",
              children: [
                {
                  name: "User.java",
                  path: "/src/main/java/com/example/User.java",
                  type: "file",
                  selected: false,
                },
                {
                  name: "Order.java",
                  path: "/src/main/java/com/example/Order.java",
                  type: "file",
                  selected: false,
                },
                {
                  name: "Product.java",
                  path: "/src/main/java/com/example/Product.java",
                  type: "file",
                  selected: false,
                },
              ],
            },
          ],
        },
      ],
    },
    {
      name: "test",
      path: "/src/test",
      type: "folder",
      children: [
        {
          name: "java",
          path: "/src/test/java",
          type: "folder",
          children: [
            {
              name: "com.example",
              path: "/src/test/java/com/example",
              type: "folder",
              children: [
                {
                  name: "UserTest.java",
                  path: "/src/test/java/com/example/UserTest.java",
                  type: "file",
                  selected: false,
                },
                {
                  name: "OrderTest.java",
                  path: "/src/test/java/com/example/OrderTest.java",
                  type: "file",
                  selected: false,
                },
              ],
            },
          ],
        },
      ],
    },
  ],
};

export interface State {
  // view mermaid-js/mermaid-live-editor/src/lib/util/serde.ts on github
  // the actual mermaid code
  code: string;
  // mermaid config
  mermaid: string;
  updateDiagram: boolean;
  rough: boolean;
}