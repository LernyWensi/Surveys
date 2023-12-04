interface ModalProps extends React.HTMLAttributes<HTMLDivElement> {
    state: boolean;
    handler: (e: React.MouseEvent<HTMLButtonElement | HTMLDivElement>) => void;
}

import style from "./Modal.module.css";

const Modal = (props: ModalProps) => {
    const { children, className, state, handler } = props;

    return (
        state && (
            <div className={style.wrapper} onClick={handler}>
                <div className={[className, style.modal].join(" ")}>{children}</div>
            </div>
        )
    );
};

export { Modal };
