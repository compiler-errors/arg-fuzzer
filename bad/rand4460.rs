
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4460(_: S1, _: S4, _: S6, _: S7, _: S8) {}
        
        fn test4460() { foo4460(S7, S1, S5, S5, S4, S6, S3); }
    