import React, { useState } from "react";
import { Button } from "../../shared/Button/Button";
import { Input } from "../../shared/Input/Input";
import styles from "./Home.module.css";
import { Question } from "./Question/Question";

interface HomeProps {
    setUserHandler: (user: null) => void;
}

const Home = (props: HomeProps) => {
    const { setUserHandler } = props;

    const [survey, setSurvey] = useState<
        {
            question: string;
            type: string;
            answers?: string[];
        }[]
    >([]);

    const [currentQuestionType, setCurrentQuestionType] = useState("");
    const [isInputActive, setIsInputActive] = useState(false);
    const [inputQuestion, setInputQuestion] = useState("");
    const [inputTitle, setInputTitle] = useState("");

    const handleLogout = async () => {
        await fetch("/api/logout", {
            method: "POST",
            headers: {
                Accept: "application/json",
            },
        }).then((res) => res.json());

        setUserHandler(null);
    };

    const handleCreate = async () => {
        if (survey.length == 0 || inputTitle.length == 0) {
            window.alert("No questions or title is missing");
            clearTitleInput();
            return;
        }

        const response = await fetch("/api/surveys", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                title: inputTitle,
                survey_data: survey,
            }),
        }).then((res) => res.json());

        if (!response.success) {
            window.alert("Something went wrong, try again");
        }

        setSurvey([]);
        clearTitleInput();
    };

    const handleAdd = () => {
        const input = inputQuestion;

        if (input.length === 0) return;

        const parsedInput = input.split("|").map((item) => item.trim());
        const question = parsedInput.shift() as string;

        const questionToAdd: {
            question: string;
            type: string;
            answers?: string[];
        } = {
            question,
            type: currentQuestionType,
        };

        if (currentQuestionType !== "input") {
            const answers = parsedInput[0].split(",").flatMap((answer) => (answer === "" ? [] : [answer.trim()]));
            questionToAdd.answers = answers;
        }

        setSurvey((prev) => [...prev, questionToAdd]);
        clearQuestionInput();
        setIsInputActive(false);
    };

    const clearQuestionInput = () => {
        setInputQuestion("");
        (document.querySelector("#question-input") as HTMLInputElement).value = "";
    };

    const clearTitleInput = () => {
        setInputTitle("");
        (document.querySelector("#title-input") as HTMLInputElement).value = "";
    };

    const handleQuestionChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setInputQuestion(e.target.value);
    };

    const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setInputTitle(e.target.value);
    };

    const handleSingle = () => {
        setIsInputActive(true);
        setCurrentQuestionType("single");
    };

    const handleMultiple = () => {
        setIsInputActive(true);
        setCurrentQuestionType("multiple");
    };

    const handleInput = () => {
        setIsInputActive(true);
        setCurrentQuestionType("input");
    };

    return (
        <div className={styles.home}>
            <div className={styles.questions}>
                {survey.map((question, index) => (
                    <Question question={question} key={index} />
                ))}
            </div>

            <div className={styles.footer}>
                <Button className={styles.logout} type="button" onClick={handleLogout}>
                    Logout
                </Button>

                <div className={styles.input_wrapper}>
                    <Input id="title-input" type="text" onChange={handleTitleChange} placeholder="Title" />

                    <Input
                        id="question-input"
                        type="text"
                        onChange={handleQuestionChange}
                        disabled={!isInputActive}
                        placeholder="How is you day? | Good, Bad, Ugly"
                    />

                    <Button type="button" onClick={handleAdd}>
                        Добавить
                    </Button>
                </div>

                <div className={styles.survey_buttons}>
                    <Button type="button" onClick={handleCreate}>
                        Создать
                    </Button>

                    <Button type="button" onClick={handleSingle}>
                        Один
                    </Button>

                    <Button type="button" onClick={handleMultiple}>
                        Несколько
                    </Button>

                    <Button type="button" onClick={handleInput}>
                        Ввод
                    </Button>
                </div>
            </div>
        </div>
    );
};

export { Home };
