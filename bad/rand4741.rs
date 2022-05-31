
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4741(_: S4, _: S7) {}
        
        fn test4741() { foo4741(S2, S2, S1, S5, S4, S3); }
    