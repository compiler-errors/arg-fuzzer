
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10881(_: S3, _: S4, _: S5, _: S7) {}
        
        fn test10881() { foo10881(S5, S6, S0, S4, S1, S3, S1); }
    