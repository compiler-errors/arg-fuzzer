
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7150(_: S6, _: S7, _: S4, _: S3, _: S1, _: S2) {}
        
        fn test7150() { foo7150(S1, S3, S4, S5, S6, S7, S8); }
    