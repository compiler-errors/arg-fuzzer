
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4485(_: S4, _: S7) {}
        
        fn test4485() { foo4485(S1, S2, S3, S4, S6, S8); }
    