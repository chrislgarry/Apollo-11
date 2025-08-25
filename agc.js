class AGCSimulator {
    constructor() {
        // Memory: 2k words of 15 bits for RAM
        this.ram = new Array(2048).fill(0);
        // 36k words of 15 bits for ROM
        this.rom = new Array(36 * 1024).fill(0);

        // Registers
        this.A = 0; // Accumulator
        this.L = 0; // Lower accumulator
        this.Q = 0; // Return address
        this.Z = 0; // Program counter
        this.EB = 0; // Erasable bank
        this.FB = 0; // Fixed bank
        this.BB = 0; // Both banks

        this.interrupts = [];
        this.io_channels = new Array(512).fill(0);
    }

    loadProgram(program) {
        const lines = program.split('\n');
        const labels = {};
        let address = 0;

        // First pass: find all labels
        for (const line of lines) {
            const trimmedLine = line.trim();
            if (trimmedLine.length === 0 || trimmedLine.startsWith('#')) {
                continue;
            }

            const parts = trimmedLine.split(/\s+/);
            const firstPart = parts[0];
            if (!this.isOpcode(firstPart) && !this.isPseudoOp(firstPart)) {
                labels[firstPart] = address;
                parts.shift();
            }

            if (parts.length > 0) {
                address++;
            }
        }

        // Second pass: parse instructions and data
        address = 0;
        for (const line of lines) {
            const trimmedLine = line.trim();
            if (trimmedLine.length === 0 || trimmedLine.startsWith('#')) {
                continue;
            }

            let parts = trimmedLine.split(/\s+/);
            if (labels.hasOwnProperty(parts[0])) {
                parts.shift();
            }

            if (parts.length === 0) continue;

            const opcode = parts[0];
            const operand = parts[1];

            if (this.isOpcode(opcode)) {
                this.rom[address] = { opcode, operand };
            } else if (this.isPseudoOp(opcode)) {
                if (opcode === 'DEC') {
                    this.rom[address] = parseInt(operand, 10);
                } else if (opcode === 'ERASE') {
                    this.rom[address] = 0; // Initialize with 0
                }
            }
            address++;
        }
        this.labels = labels;
    }

    isOpcode(s) {
        return ['CA', 'AD', 'TS', 'TCF'].includes(s);
    }

    isPseudoOp(s) {
        return ['DEC', 'ERASE'].includes(s);
    }

    step() {
        const instruction = this.rom[this.Z];
        if (!instruction) {
            return; // Halt if no instruction
        }

        const { opcode, operand } = instruction;

        switch (opcode) {
            case 'CA':
                this.A = this.getValue(operand);
                this.Z++;
                break;
            case 'AD':
                this.A += this.getValue(operand);
                this.Z++;
                break;
            case 'TS':
                this.setValue(operand, this.A);
                this.Z++;
                break;
            case 'TCF':
                this.Z = this.labels[operand];
                break;
            default:
                // Unrecognized instruction, halt or throw error
                console.error(`Unrecognized instruction: ${opcode}`);
                return;
        }
    }

    getValue(operand) {
        if (this.labels.hasOwnProperty(operand)) {
            const address = this.labels[operand];
            return this.rom[address];
        }
        return parseInt(operand, 10);
    }

    setValue(operand, value) {
        if (this.labels.hasOwnProperty(operand)) {
            const address = this.labels[operand];
            this.rom[address] = value;
        } else {
            console.error(`Unknown label: ${operand}`);
        }
    }
}
