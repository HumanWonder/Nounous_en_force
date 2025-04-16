// Permet de naviguer dans le formulaire

interface StepControlsProps {
    currentStep: number;
    totalSteps: number;
    onBack: () => void;
    onNext: () => void;
    onSubmit?: (e:any) => void; //Optionnel pour qu'il soit appliqué qu'à la dernière étape
}

export const StepControls = ({
    currentStep,
    totalSteps,
    onBack,
    onNext,
    onSubmit,
}: StepControlsProps) => {
    return (
        <div className="flex justify-between mt-6">
            {currentStep > 0 ? (
                <button type="button" onClick={onBack} className="btn-secondary">
                    Précédent
                </button>
            ) : (
                <div />
            )}
            {currentStep < totalSteps - 1 ? (
                <button type="button" onClick={onNext} className="btn-primary">
                    Suivant
                </button>
            ) : (
                <button type="button" onClick={onSubmit} className="btn-primary">
                    Valider
                </button>
            )}
        </div>
    );
};