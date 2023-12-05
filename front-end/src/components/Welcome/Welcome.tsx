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
                Добро пожаловать на <span>Опросы</span>
            </h1>
            <div className={styles.button_container}>
                <Button onClick={toggleLoginModal}>Login</Button>
                <Button onClick={toggleSignupModal}>Sign Up</Button>
            </div>
            <div className={styles.hint}>
                <p>
                    Вы не можете <span>создавать опросы</span> до того, как <span>авторизуетесь</span>.
                </p>
                <p>
                    <span>Зарегистрируйтесь</span> и попробуйте создать ваш первый опрос.
                </p>
                <p>
                    Вы можете участвовать в <span>опросах</span>, используя заранее представленный <span>ID </span>
                    <span>в поисковой строке,</span> в заголовке страницы.
                </p>
            </div>
        </div>
    );
};

export { Welcome };
