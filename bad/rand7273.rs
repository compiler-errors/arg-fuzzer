
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7273(_: S1, _: S2, _: S5, _: S7, _: S8) {}
        
        fn test7273() { foo7273(S1, S4, S1, S5, S3, S3); }
    