
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4271(_: S6, _: S8, _: S1, _: S5, _: S2, _: S7, _: S4, _: S3) {}
        
        fn test4271() { foo4271(S0, S0, S2, S4, S5); }
    