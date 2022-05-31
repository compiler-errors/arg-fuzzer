
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6834(_: S1, _: S3, _: S7, _: S8) {}
        
        fn test6834() { foo6834(S5, S5, S7, S1, S4, S1); }
    