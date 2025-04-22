//Barre de progression d'étapes pour le formulaire

interface StepProgressBarProps {
    steps: string[];
    currentStep: number;
}

export const StepProgressBar = ({ steps, currentStep }: StepProgressBarProps) => {
    const percentage = ((currentStep + 1) / steps.length) * 100;

    return (
        <div className="w-full mb-4">
            <div className="flex justify-between text-sm text-gray-600 mb-1">
                {steps.map((step, index) => (
                    <div
                        key={index}
                        className={`w-full text-center ${index === currentStep ? "font-bold text-blue-600" : ""
                            }`}
                    >
                        {index + 1}
                    </div>
                ))}
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                    className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                    style={{ width: `${percentage}%` }}
                />
            </div>
            <p className="text-center text-xs mt-1 text-gray-500">
                Étape {currentStep + 1} / {steps.length} — {steps[currentStep]}
            </p>
        </div>
    );
};
