const mockCourseData = {
    COMP3431: {
        code: "COMP3431",
        name: "Robotic Software Architecture",
        UOC: 6,
        description: "An introduction to Intelligent agent design. Picking actions using planning, \ " +
            "learning or engineered control. Both practical and theoretical components. ",
        conditions: ["COMP2521 or COMP1927","WAM of at least 70"],
        offerterms: ["Term3"]

    },
    COMP1511: {
        code: "COMP1511",
        name: "Introduction to Programming",
        UOC: 6,
        description: "This course assumes no previous programming knowledge, \ " +
            "and will introduce you to the basics of programming in Python.",
        conditions: [],
        offerterms: ["Term1","Term2","Term3"]
    },
    COMP1521: {
        code: "COMP1521",
        name: "Computer Systems Fundamentals",
        UOC: 6,
        description: "This course will introduce you to the fundamental concepts of computer systems, \ " +
            "including low-level programming, operating systems, computer networks, and embedded systems.",
        conditions: ["COMP1511"],
        offerterms: ["Term1","Term2","Term3"]
    },
    COMP1531: {
        code: "COMP1531",
        name: "Software Engineering Fundamentals",
        UOC: 6,
        description: "This course will introduce you to the fundamental concepts of software engineering, \ " +
            "including software design, quality assurance, and project management.",
        conditions: ["COMP1511"],
        offerterms: ["Term1","Term2","Term3"]
    }
};

import { handbook_interface } from "src/data/data.service";

export const CourseInterface = {
    async getCourseInfo(CourseCode: string) {

        // 模拟异步行为
        //await new Promise(resolve => setTimeout(resolve, 100)); // 模拟延时
        return mockCourseData[CourseCode] || null;
        
        // TODO: Handel the case when the CourseCode is not found, i.e. get_course_info return `undefined`
        
        return handbook_interface.get_course_info(CourseCode)
    },


};
