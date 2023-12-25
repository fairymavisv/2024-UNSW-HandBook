const mockProgramData = {
    3778: {
        name: "Computer Science",
        UOC: 144,
        description: "Computer Science involves the study of the design, construction and uses of computer systems.\ " +
            "It is concerned with the representation of data and data structures in computer systems",
        majorList: [
            {
                name: "Computer Science",
                UOC: 96,
                description:"Computers are now ubiquitous, and critical to the functioning of all enterprises in modern industrial societies, \f" +
                    "rom commerce to health and education.",
                CompulsoryCourseList: ["COMP1511", "COMP1521","COMP1531","COMP2511"],
                SpecializedElectiveCourses: []
            },
            {   name: "Computer Science (Database Systems)",
                UOC: 96,
                description:"Database management systems (DBMSs) provide essential infrastructure for handling the large volumes of data required by \ " +
                    "modern enterprises: corporate, government, scientific and educational.",
                CompulsoryCourseList:["COMP1511", "COMP1521","COMP1531","COMP2511"],
                SpecializedElectiveCourses: ["COMP6714","COMP9312","COMP9315","COMP9319","comp9313"]
            }
        ],
        CompulsoryCourseList: [],
        SpecializedElectiveCourses: []
    },
    // ...其他程序数据
};

export const programInterface = {
    async getProgramInfo(programCode: string) {
        // 模拟异步行为
        //await new Promise(resolve => setTimeout(resolve, 100)); // 模拟延时
        return mockProgramData[programCode] || null;
    },

    async getMajorInfo(programCode: string, majorName: string) {

        const program = mockProgramData[programCode];
        if (program) {
            return program.majorList.find(major => major.name === majorName) || null;
        }
        return null;
    }
};
