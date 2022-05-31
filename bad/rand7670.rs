
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7670(_: S6, _: S3, _: S7, _: S2) {}
        
        fn test7670() { foo7670(S3, S4, S6, S7, S8); }
    