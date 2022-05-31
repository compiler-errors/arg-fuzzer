
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11049(_: S1, _: S4, _: S6) {}
        
        fn test11049() { foo11049(S7, S6, S0, S7, S7, S1, S0); }
    