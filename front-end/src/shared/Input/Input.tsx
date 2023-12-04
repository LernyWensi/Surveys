interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {}

import styles from "./Input.module.css";

const Input = (props: InputProps) => {
    const { id, type, placeholder, className, value, onChange, disabled } = props;

    return (
        <input
            id={id}
            type={type}
            placeholder={placeholder}
            value={value}
            onChange={onChange}
            disabled={disabled}
            className={[className, styles.input].join(" ")}
        />
    );
};

export { Input };
