interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {}

import style from "./Button.module.css";

const Button = (props: ButtonProps) => {
    const { children, onClick, type, className } = props;

    return (
        <button className={[className, style.button].join(" ")} onClick={onClick} type={type}>
            {children}
        </button>
    );
};

export { Button };
