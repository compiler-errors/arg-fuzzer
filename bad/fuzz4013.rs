
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4013(_: S1, _: S3, _: S6, _: S7) {}
        
        fn test4013() { foo4013(S2, S1, S4, S6, S5); }
    