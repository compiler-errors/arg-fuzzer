
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7855(_: S7, _: S3, _: S6, _: S5, _: S2) {}
        
        fn test7855() { foo7855(S1, S2, S3, S4, S6, S7, S8); }
    