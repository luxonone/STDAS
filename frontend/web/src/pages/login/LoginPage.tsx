import { type FormEvent, useState } from "react";
import { login, type LoginCredentials, type LoginResult } from "../../shared/api";

type LoginState =
  | { kind: "idle" }
  | { kind: "submitting" }
  | { kind: "error"; message: string };

interface LoginPageProps {
  onAuthenticated: (result: LoginResult) => void;
}

function getErrorMessage(error: unknown) {
  return error instanceof Error
    ? error.message
    : "Unable to sign in. Check your account and password.";
}

export function LoginPage({ onAuthenticated }: LoginPageProps) {
  const [credentials, setCredentials] = useState<LoginCredentials>({
    password: "",
    username: ""
  });
  const [state, setState] = useState<LoginState>({ kind: "idle" });

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setState({ kind: "submitting" });

    try {
      const result = await login(credentials);
      onAuthenticated(result);
    } catch (error) {
      setState({ kind: "error", message: getErrorMessage(error) });
    }
  }

  const isSubmitting = state.kind === "submitting";

  return (
    <main className="login-page" aria-label="STDAS login">
      <section className="login-hero" aria-label="STDAS brand">
        <img
          alt="STDAS Test Data Analytics"
          className="login-hero__logo"
          src="/login-assets/logos/stdas-hero-logo.png"
        />
        <div className="login-hero__support" aria-hidden="true">
          <span />
          <span />
          <span />
        </div>
      </section>

      <section className="login-card" aria-label="Sign in">
        <header className="login-card__identity">
          <img
            alt=""
            aria-hidden="true"
            className="login-card__icon"
            src="/login-assets/logos/stdas-icon.png"
          />
          <div>
            <h1>Welcome back</h1>
            <p>STDAS internal analytics workspace</p>
          </div>
        </header>

        <form className="login-form" onSubmit={handleSubmit}>
          <div className="login-form__divider" />

          <label className="login-field">
            <span>Factory Account</span>
            <span className="login-input">
              <img alt="" aria-hidden="true" src="/login-assets/icons/user.svg" />
              <input
                autoComplete="username"
                name="username"
                onChange={(event) =>
                  setCredentials((current) => ({
                    ...current,
                    username: event.target.value
                  }))
                }
                placeholder="Enter your account"
                required
                type="text"
                value={credentials.username}
              />
            </span>
          </label>

          <label className="login-field">
            <span>Password</span>
            <span className="login-input">
              <img alt="" aria-hidden="true" src="/login-assets/icons/lock.svg" />
              <input
                autoComplete="current-password"
                name="password"
                onChange={(event) =>
                  setCredentials((current) => ({
                    ...current,
                    password: event.target.value
                  }))
                }
                placeholder="Enter your password"
                required
                type="password"
                value={credentials.password}
              />
              <img
                alt=""
                aria-hidden="true"
                className="login-input__trailing"
                src="/login-assets/icons/eye.svg"
              />
            </span>
          </label>

          <label className="login-remember">
            <input type="checkbox" />
            <span>Remember account</span>
          </label>

          <button className="login-submit" disabled={isSubmitting} type="submit">
            {isSubmitting ? "Signing in" : "Sign in"}
          </button>

          {state.kind === "error" ? (
            <p className="login-error" role="alert">
              {state.message}
            </p>
          ) : null}

          <a className="login-forgot" href="#forgot-password">
            Forgot password?
          </a>

          <section className="login-security" aria-label="Security notice">
            <img
              alt=""
              aria-hidden="true"
              className="login-security__icon"
              src="/login-assets/icons/shield-check.svg"
            />
            <div>
              <h2>Secure internal system</h2>
              <p>
                This system is for authorized factory personnel only.
                <br />
                All activities are logged and monitored.
                <br />
                Do not share your account or password.
              </p>
            </div>
          </section>
        </form>
      </section>
    </main>
  );
}
