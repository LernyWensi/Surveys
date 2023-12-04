import { useState } from "react";
import { Button } from "../../../shared/Button/Button";
import { Input } from "../../../shared/Input/Input";
import { Modal } from "../../../shared/Modal/Modal";

import { User } from "../../../App";
import styles from "./LoginModal.module.css";

interface LoginModalProps {
    setUserHandler: (user: User) => void;
    loginIsOpen: boolean;
    toggleLoginModal: (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => void;
}

const LoginModal = (props: LoginModalProps) => {
    const { loginIsOpen, toggleLoginModal, setUserHandler } = props;

    const [name, setName] = useState("");
    const [password, setPassword] = useState("");

    const handleNameInput = (e: React.ChangeEvent<HTMLInputElement>) => {
        setName(e.target.value);
    };

    const handlePasswordInput = (e: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(e.target.value);
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const userForLogin = {
            name,
            password,
        };

        const response = await fetch("/api/login", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(userForLogin),
        }).then((res) => res.json());

        if (response.success) {
            setUserHandler(response.data.user);
        }
    };

    return (
        <Modal state={loginIsOpen} handler={toggleLoginModal}>
            <form className={styles.login} onSubmit={handleSubmit}>
                <label htmlFor="name">Name</label>
                <Input type="text" placeholder="Did I know you?" name="name" onChange={handleNameInput} />
                <label htmlFor="password">Password</label>
                <Input type="password" placeholder="Did I know this?" name="password" onChange={handlePasswordInput} />
                <Button type="submit">Login</Button>
            </form>
        </Modal>
    );
};

export { LoginModal };
