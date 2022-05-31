
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10484(_: S0, _: S6, _: S3, _: S1) {}
        
        fn test10484() { foo10484(S1, S4, S5, S6, S7, S8); }
    