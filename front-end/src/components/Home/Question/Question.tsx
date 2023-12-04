interface QuestionProps {
    question: { question: string; type: string; answers?: string[] };
}

import styles from "./Question.module.css";

const Question = (props: QuestionProps) => {
    const { question } = props;

    return (
        <div className={styles.question}>
            <p>
                <span className={styles.tag}>Question: </span>
                {question.question}
            </p>
            <p>
                <span className={styles.tag}>Type: </span>
                {question.type}
            </p>
            {question.answers && (
                <p>
                    <span className={styles.tag}>Answers: </span>
                    {question.answers?.join(", ")}
                </p>
            )}
        </div>
    );
};

export { Question };
