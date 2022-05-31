
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7523(_: S5, _: S4, _: S1, _: S4, _: S3) {}
        
        fn test7523() { foo7523(S6, S8, S1, S4, S7, S5, S2); }
    