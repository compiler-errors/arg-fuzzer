
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7928(_: S2, _: S7) {}
        
        fn test7928() { foo7928(S4, S2, S5); }
    