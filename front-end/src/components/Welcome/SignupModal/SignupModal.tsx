import React, { useState } from "react";
import { Button } from "../../../shared/Button/Button";
import { Input } from "../../../shared/Input/Input";
import { Modal } from "../../../shared/Modal/Modal";

import { User } from "../../../App";
import styles from "./SignupModal.module.css";

interface SignupModalProps {
    setUserHandler: (user: User) => void;
    signupIsOpen: boolean;
    toggleSignupModal: (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => void;
}

const SignupModal = (props: SignupModalProps) => {
    const { signupIsOpen, toggleSignupModal, setUserHandler } = props;

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

        const userForCreate = {
            name,
            password,
        };

        const response = await fetch("/api/signup", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(userForCreate),
        }).then((res) => res.json());

        if (response.success) {
            await fetch("/api/login", {
                method: "POST",
                headers: {
                    Accept: "application/json",
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(userForCreate),
            }).then((res) => res.json());

            response.success && setUserHandler(response.data.user);
        }
    };

    return (
        <Modal state={signupIsOpen} handler={toggleSignupModal}>
            <form onSubmit={handleSubmit} className={styles.signup}>
                <label htmlFor="name">Name</label>
                <Input type="text" placeholder="Hi, first time there?" name="name" onChange={handleNameInput} />
                <label htmlFor="password">Password</label>
                <Input type="password" placeholder="Make it strong" name="password" onChange={handlePasswordInput} />
                <Button type="submit">Sign Up</Button>
            </form>
        </Modal>
    );
};

export { SignupModal };
