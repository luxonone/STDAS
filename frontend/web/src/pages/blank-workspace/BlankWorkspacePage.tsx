import type { StoredSession } from "../../shared/auth/session";

interface BlankWorkspacePageProps {
  onSignOut: () => void;
  session: StoredSession;
}

export function BlankWorkspacePage({
  onSignOut,
  session
}: BlankWorkspacePageProps) {
  return (
    <main className="blank-workspace" aria-label="STDAS workspace placeholder">
      <header className="blank-workspace__header">
        <div>
          <p className="blank-workspace__eyebrow">STDAS workspace</p>
          <h1>登录成功</h1>
        </div>
        <button onClick={onSignOut} type="button">
          Sign out
        </button>
      </header>

      <section className="blank-workspace__body">
        <p>
          当前登录用户：<strong>{session.displayName}</strong>
        </p>
        <p>
          登录链路已经接通。这里暂时保留为空白工作区，等待下一张页面设计稿确认后继续实现。
        </p>
      </section>
    </main>
  );
}
