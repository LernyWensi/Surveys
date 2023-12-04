import React, { useState } from "react";
import { Input } from "../../shared/Input/Input";

import { Button } from "../../shared/Button/Button";
import { Modal } from "../../shared/Modal/Modal";
import styles from "./SurveySearchBar.module.css";

const SurveySearchBar = () => {
    const [surveyId, setSurveyId] = useState("");
    const [isSurveyOpen, setIsSurveyOpen] = useState(false);
    const [surveyData, setSurveyData] = useState([]);

    const toggleSurveyModal = (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => {
        if (e.target !== e.currentTarget) return;

        setIsSurveyOpen((prev) => !prev);
        setSurveyData([]);
        clearSearchBar();
    };

    const handleSurveyIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSurveyId(e.target.value);
    };

    const handleSearch = async () => {
        const response = await fetch(`/api/surveys/${surveyId}`, {
            method: "GET",
            headers: {
                Accept: "application/json",
            },
        })
            .then((res) => res.json())
            .catch(() => {
                window.alert("Invalid survey ID (should be UUID) or Survey doesn't exist");
            });

        if (response.success) {
            setSurveyData(response.data.survey["survey_data"]);
        }

        setIsSurveyOpen(true);
    };

    const clearSearchBar = () => {
        (document.querySelector("#search-bar") as HTMLInputElement).value = "";
        setSurveyId("");
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const dataToSubmit: { survey_id: string; result_data: object[] } = {
            survey_id: surveyId,
            result_data: [],
        };

        const surveyForm = e.currentTarget;

        for (const element of surveyForm.children) {
            if (!(element instanceof HTMLDivElement)) break;

            const answer: { type: string; question: string; answers: string[] | null } = {
                type: element.dataset.questionType as string,
                question: element.dataset.question as string,
                answers: null,
            };

            if (answer.type === "input") {
                answer.answers = [(element.querySelector("input") as HTMLInputElement).value];
            }

            if (answer.type === "multiple") {
                answer.answers = Array.from(element.querySelectorAll("input:checked")).map(
                    (e) => (e as HTMLInputElement).value
                );
            }

            if (answer.type === "single") {
                answer.answers = Array.from(element.querySelectorAll("input:checked")).map(
                    (e) => (e as HTMLInputElement).value
                );
            }

            dataToSubmit.result_data.push(answer);
        }

        const response = await fetch("/api/results", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(dataToSubmit),
        }).then((res) => res.json());

        if (response.success) {
            setSurveyData([]);
            setIsSurveyOpen(false);
            setSurveyId("");
            clearSearchBar();
        }
    };

    return (
        <div className={styles.wrapper}>
            <Input
                id="search-bar"
                className={styles.input}
                type="text"
                placeholder="Search for survey by ID"
                onChange={handleSurveyIdChange}
            />
            <Button className={styles.search} type="button" onClick={handleSearch}>
                Search
            </Button>

            <Modal state={isSurveyOpen} handler={toggleSurveyModal} className={styles.modal}>
                <form onSubmit={handleSubmit}>
                    {surveyData.map(
                        ({ question, type, answers }: { question: string; type: string; answers: string[] }, index) => (
                            <div
                                className={styles.question}
                                key={index}
                                data-question-type={type}
                                data-question={question}
                            >
                                <p className={styles.question_title}>{question}</p>

                                {type === "single" && (
                                    <div className={styles.question_body}>
                                        {answers.map((answer, key) => (
                                            <div key={key}>
                                                <input
                                                    type="radio"
                                                    id={answer}
                                                    value={answer}
                                                    name={index.toString()}
                                                />
                                                <label htmlFor={answer}>{answer}</label>
                                            </div>
                                        ))}
                                    </div>
                                )}

                                {type === "multiple" && (
                                    <div className={styles.question_body}>
                                        {answers.map((answer, key) => (
                                            <div key={key}>
                                                <input
                                                    type="checkbox"
                                                    id={answer}
                                                    value={answer}
                                                    name={index.toString()}
                                                />
                                                <label htmlFor={answer}>{answer}</label>
                                            </div>
                                        ))}
                                    </div>
                                )}

                                {type === "input" && (
                                    <div className={styles.question_body}>
                                        <Input type="text" />
                                    </div>
                                )}
                            </div>
                        )
                    )}

                    <Button type="submit">Submit</Button>
                </form>
            </Modal>
        </div>
    );
};

export { SurveySearchBar };
