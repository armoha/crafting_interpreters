// Doubly-linked list for malloc string
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Node
{
    struct Node *prev;
    struct Node *next;
    char *data;
} Node;

Node *head;
Node *tail;
unsigned int count;

Node *new (char *data)
{
    Node *new_node = (Node *)malloc(sizeof(Node));
    new_node->prev = NULL;
    new_node->next = NULL;
    new_node->data = data;
    count += 1;
    return new_node;
}

void prepend(char *data)
{
    Node *new_node = new (data);
    if (head == NULL)
    {
        head = new_node;
        if (tail == NULL)
            tail = new_node;
        return;
    }
    new_node->next = head;
    head->prev = new_node;
    head = new_node;
}

void append(char *data)
{
    Node *new_node = new (data);
    if (tail == NULL)
    {
        tail = new_node;
        if (head == NULL)
            head = new_node;
        return;
    }
    new_node->prev = tail;
    tail->next = new_node;
    tail = new_node;
}

// stdio has int __cdecl remove(const char *_Filename);
void remove_node(Node *node)
{
    if (node == NULL)
        return;
    if (head == node)
        head = node->next;
    if (tail == node)
        tail = node->prev;
    if (node->prev != NULL)
        node->prev->next = node->next;
    if (node->next != NULL)
        node->next->prev = node->prev;
    free(node);
    count -= 1;
}

char *pop_front()
{
    char *data = head->data;
    remove_node(head);
    return data;
}

char *pop_back()
{
    char *data = tail->data;
    remove_node(tail);
    return data;
}

_Bool contain(char *data)
{
    Node *temp = head;
    while (temp != NULL)
    {
        if (strcmp(temp->data, data) == 0)
            return 1;
        temp = temp->next;
    }
    return 0;
}

void print_all()
{
    Node *temp = head;
    printf("print: ");
    while (temp != NULL)
    {
        printf("%s ", temp->data);
        temp = temp->next;
    }
    printf("\n");
}

void rev_print_all()
{
    Node *temp = tail;
    printf("rev_print: ");
    while (temp != NULL)
    {
        printf("%s ", temp->data);
        temp = temp->prev;
    }
    printf("\n");
}

#define push_front(s)             \
    do                            \
    {                             \
        temp = malloc(sizeof(s)); \
        strcpy(temp, s);          \
        prepend(temp);            \
    } while (0)

#define push_back(s)              \
    do                            \
    {                             \
        temp = malloc(sizeof(s)); \
        strcpy(temp, s);          \
        append(temp);             \
    } while (0)

int main()
{
    head = NULL;
    tail = NULL;
    char *temp;

    push_front("test\0");
    push_front("exercise\0");
    push_back("list\0");
    print_all();

    temp = pop_front();
    printf("pop_front: %s\t", temp);
    print_all();

    printf("%s\n", contain("test") ? "true" : "false");
    temp = pop_back();
    printf("pop_back: %s\t", temp);
    print_all();
    printf("%s\n", contain("list") ? "true" : "false");
}