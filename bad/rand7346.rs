
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7346(_: S2, _: S3, _: S8) {}
        
        fn test7346() { foo7346(S7, S6, S3, S1); }
    