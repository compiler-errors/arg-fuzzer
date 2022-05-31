
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4558(_: S2, _: S2, _: S4, _: S1, _: S2) {}
        
        fn test4558() { foo4558(S7, S3, S4, S6, S1, S1, S2); }
    