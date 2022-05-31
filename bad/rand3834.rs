
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3834(_: S2, _: S3, _: S6, _: S8) {}
        
        fn test3834() { foo3834(S5, S2, S0, S3, S1); }
    