
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3783(_: S1, _: S3, _: S4, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test3783() { foo3783(S6, S8, S8, S8, S6, S4, S8, S6); }
    