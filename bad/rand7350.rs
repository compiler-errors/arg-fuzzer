
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7350(_: S7, _: S7, _: S3, _: S2, _: S3) {}
        
        fn test7350() { foo7350(S1, S2, S3, S5, S7, S8); }
    