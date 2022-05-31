
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16152(_: S2, _: S5, _: S4) {}
        
        fn test16152() { foo16152(S1, S2, S4, S6, S7, S8); }
    