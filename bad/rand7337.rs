
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7337(_: S3, _: S5, _: S7, _: S4) {}
        
        fn test7337() { foo7337(S3, S2, S5, S4, S1, S5); }
    