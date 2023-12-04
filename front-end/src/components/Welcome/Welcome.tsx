import { useState } from "react";
import { User } from "../../App";
import { Button } from "../../shared/Button/Button";
import { LoginModal } from "./LoginModal/LoginModal";
import { SignupModal } from "./SignupModal/SignupModal";
import styles from "./Welcome.module.css";

interface WelcomeProps {
    setUserHandler: (user: User) => void;
}

const Welcome = (props: WelcomeProps) => {
    const { setUserHandler } = props;

    const [loginIsOpen, setLoginIsOpen] = useState(false);
    const [signupIsOpen, setSignupIsOpen] = useState(false);

    const toggleLoginModal = (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => {
        if (e.target !== e.currentTarget) return;
        setLoginIsOpen((prev) => !prev);
    };

    const toggleSignupModal = (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => {
        if (e.target !== e.currentTarget) return;
        setSignupIsOpen((prev) => !prev);
    };

    return (
        <div className={styles.wrapper}>
            <LoginModal setUserHandler={setUserHandler} loginIsOpen={loginIsOpen} toggleLoginModal={toggleLoginModal} />
            <SignupModal
                setUserHandler={setUserHandler}
                signupIsOpen={signupIsOpen}
                toggleSignupModal={toggleSignupModal}
            />

            <h1 className={styles.title}>
                Welcome to <span>Surveys</span>
            </h1>
            <div className={styles.button_container}>
                <Button onClick={toggleLoginModal}>Login</Button>
                <Button onClick={toggleSignupModal}>Sign Up</Button>
            </div>
            <div className={styles.hint}>
                <p>
                    You can't create <span>surveys</span> before you <span>logged in</span>. Consider{" "}
                    <span>sign up</span> and try to create your first survey.
                </p>
                <p>
                    You can patriciate in <span>survey</span> by searching <span>it</span> by id in the{" "}
                    <span>search bar</span> at the top.
                </p>
            </div>
        </div>
    );
};

export { Welcome };
