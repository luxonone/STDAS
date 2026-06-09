import { useState } from "react";
import type { StoredSession } from "../../shared/auth/session";

interface DataExplorerPageProps {
  onSignOut: () => void;
  session: StoredSession;
}

type FrameKey =
  | "default"
  | "filtered"
  | "filteredScroll"
  | "importEmpty"
  | "importResolved"
  | "importPackage"
  | "notifications";

interface DesignFrame {
  alt: string;
  src: string;
}

interface Hotspot {
  action: () => void;
  ariaLabel: string;
  height: number;
  left: number;
  title?: string;
  top: number;
  width: number;
}

const DESIGN_FRAME_WIDTH = 1672;
const DESIGN_FRAME_HEIGHT = 941;

const DESIGN_FRAMES: Record<FrameKey, DesignFrame> = {
  default: {
    alt: "STDAS Data Explorer default empty state from Pencil design",
    src: "/pencil-assets/data-explorer/bi8Au.png"
  },
  filtered: {
    alt: "STDAS Data Explorer filtered lot list state from Pencil design",
    src: "/pencil-assets/data-explorer/e12ug.png"
  },
  filteredScroll: {
    alt: "STDAS Data Explorer 50 rows per page scroll state from Pencil design",
    src: "/pencil-assets/data-explorer/NJBmx.png"
  },
  importEmpty: {
    alt: "STDAS Data Explorer import data empty input drawer from Pencil design",
    src: "/pencil-assets/data-explorer/iAbdh.png"
  },
  importResolved: {
    alt: "STDAS Data Explorer import data parser profile resolved drawer from Pencil design",
    src: "/pencil-assets/data-explorer/knat2.png"
  },
  importPackage: {
    alt: "STDAS Data Explorer import data package selected drawer from Pencil design",
    src: "/pencil-assets/data-explorer/wDbbz.png"
  },
  notifications: {
    alt: "STDAS Data Explorer notifications open state from Pencil design",
    src: "/pencil-assets/data-explorer/K0ezH.png"
  }
};

const LIST_FRAMES = new Set<FrameKey>(["default", "filtered", "filteredScroll"]);
const IMPORT_FRAMES = new Set<FrameKey>(["importEmpty", "importResolved", "importPackage"]);

export function DataExplorerPage({
  onSignOut
}: DataExplorerPageProps) {
  const [frame, setFrame] = useState<FrameKey>("default");
  const [returnFrame, setReturnFrame] = useState<FrameKey>("default");
  const currentFrame = DESIGN_FRAMES[frame];
  const hotspots = buildHotspots({
    closeOverlay: () => setFrame(returnFrame),
    frame,
    openImport: () => {
      setReturnFrame(LIST_FRAMES.has(frame) ? frame : "default");
      setFrame("importEmpty");
    },
    openNotifications: () => {
      setReturnFrame(IMPORT_FRAMES.has(frame) || frame === "notifications" ? returnFrame : frame);
      setFrame("notifications");
    },
    reset: () => setFrame("default"),
    selectImportProfile: () => setFrame("importResolved"),
    selectImportPackage: () => setFrame("importPackage"),
    search: () => setFrame("filtered"),
    showFiftyPerPage: () => setFrame("filteredScroll"),
    signOut: onSignOut
  });

  return (
    <main className="data-explorer" aria-label="STDAS Data Explorer">
      <section
        className="data-explorer__pencil-stage"
        style={{
          height: DESIGN_FRAME_HEIGHT,
          width: DESIGN_FRAME_WIDTH
        }}
      >
        <img
          alt={currentFrame.alt}
          className="data-explorer__pencil-frame"
          draggable={false}
          height={DESIGN_FRAME_HEIGHT}
          src={currentFrame.src}
          width={DESIGN_FRAME_WIDTH}
        />

        {hotspots.map((hotspot) => (
          <button
            aria-label={hotspot.ariaLabel}
            className="data-explorer__hotspot"
            key={hotspot.ariaLabel}
            onClick={hotspot.action}
            style={{
              height: hotspot.height,
              left: hotspot.left,
              top: hotspot.top,
              width: hotspot.width
            }}
            title={hotspot.title ?? hotspot.ariaLabel}
            type="button"
          />
        ))}

        <div aria-hidden="true" className="data-explorer__preload">
          {Object.entries(DESIGN_FRAMES)
            .filter(([key]) => key !== frame)
            .map(([key, designFrame]) => (
              <img alt="" key={key} src={designFrame.src} />
            ))}
        </div>
      </section>
    </main>
  );
}

function buildHotspots(actions: {
  closeOverlay: () => void;
  frame: FrameKey;
  openImport: () => void;
  openNotifications: () => void;
  reset: () => void;
  search: () => void;
  selectImportPackage: () => void;
  selectImportProfile: () => void;
  showFiftyPerPage: () => void;
  signOut: () => void;
}) {
  const hotspots: Hotspot[] = [
    {
      action: actions.openNotifications,
      ariaLabel: "Open notifications",
      height: 76,
      left: 1186,
      top: 0,
      width: 72
    },
    {
      action: actions.signOut,
      ariaLabel: "Sign out",
      height: 52,
      left: 1628,
      top: 12,
      width: 44
    }
  ];

  if (LIST_FRAMES.has(actions.frame)) {
    hotspots.push(
      {
        action: actions.search,
        ariaLabel: "Search lots",
        height: 36,
        left: 1256,
        top: 134,
        width: 128
      },
      {
        action: actions.reset,
        ariaLabel: "Reset filters",
        height: 36,
        left: 1384,
        top: 134,
        width: 122
      },
      {
        action: actions.openImport,
        ariaLabel: "Open import data",
        height: 38,
        left: 1500,
        top: 178,
        width: 146
      },
      {
        action: actions.showFiftyPerPage,
        ariaLabel: "Show 50 rows per page state",
        height: 34,
        left: 1152,
        top: 244,
        width: 72
      }
    );
  }

  if (actions.frame === "notifications") {
    hotspots.push({
      action: actions.closeOverlay,
      ariaLabel: "Close notifications",
      height: 34,
      left: 1388,
      top: 100,
      width: 58
    });
  }

  if (actions.frame === "importEmpty") {
    hotspots.push(
      {
        action: actions.closeOverlay,
        ariaLabel: "Close import data",
        height: 48,
        left: 1606,
        top: 90,
        width: 48
      },
      {
        action: actions.selectImportProfile,
        ariaLabel: "Select AC STS8200 import profile",
        height: 72,
        left: 1222,
        top: 180,
        width: 426
      }
    );
  }

  if (actions.frame === "importResolved") {
    hotspots.push(
      {
        action: actions.closeOverlay,
        ariaLabel: "Close import data",
        height: 48,
        left: 1606,
        top: 90,
        width: 48
      },
      {
        action: actions.selectImportPackage,
        ariaLabel: "Select import package",
        height: 126,
        left: 1214,
        top: 420,
        width: 432
      }
    );
  }

  if (actions.frame === "importPackage") {
    hotspots.push({
      action: actions.closeOverlay,
      ariaLabel: "Close import data",
      height: 48,
      left: 1606,
      top: 90,
      width: 48
    });
  }

  return hotspots;
}
