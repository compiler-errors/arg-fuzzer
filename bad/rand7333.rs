
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7333(_: S4, _: S2) {}
        
        fn test7333() { foo7333(S1, S4, S6, S8); }
    