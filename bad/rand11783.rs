
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11783(_: S1, _: S2, _: S6, _: S7, _: S8) {}
        
        fn test11783() { foo11783(S2, S2, S0, S2, S3, S1, S2); }
    