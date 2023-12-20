// program.model.ts

export class Program {
    constructor(public name: string, public UOC: number, public code: string, public majorList: Major[],public CompulsoryCourseList: Course[],public SpecializedElectiveCourses: Course[] ) {}
}


export class Major {
    constructor(public name: string, public UOC: number, public CompulsoryCourseList: Course[],public SpecializedElectiveCourses: Course[]) {}
}

export class Course {
    constructor(public code: string, public name: string, public uoc: number) {}
}